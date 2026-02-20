# ADCS
# Copyright (c) 2026 Joseph Hobbs

import time

from matplotlib import pyplot as plt
import numpy as np

from adcs import (
    AngularMomentum,
    AngularVelocity,
    Inertia,
    KaneDamper,
    Quaternion,
    State,
    Torque,
)
from adcs import integrators as it

plt.rcParams["font.family"] = "Poppins"
plt.rcParams["font.size"]   = 12

INERTIA = 0.1
DAMPING = 0.1

DT = 5e-4
TIME = 20
ITERATIONS = int(TIME // DT)

if __name__ == "__main__":
    # Initialize state
    inertia = Inertia(1, 1.05, 2, 0, 0, 0)
    state = State(inertia)

    # Set damping
    state.damper = KaneDamper(
        INERTIA,
        DAMPING,    
    )
    
    # Set angular velocity
    state.angular_velocity = AngularVelocity(2, 0.3, 0.3)

    # Integrate
    integrator = it.RungeKutta4(DT)
    n = Quaternion.from_vector(-1, 0, 0)
    w = Quaternion.from_vector(0, 1, 0)
    nose = []
    wingtip = []
    angular_momenta = []
    start = time.perf_counter()
    for _ in range(ITERATIONS):
        state = integrator.step(state)
        nose.append(np.array((state.quaternion * n * state.quaternion.inv()).vector))
        wingtip.append(np.array((state.quaternion * w * state.quaternion.inv()).vector))

        # Bus angular momentum
        h = AngularMomentum.product(
            state.inertia,
            state.angular_velocity,
        )

        # Damper angular momentum
        hd = AngularMomentum.product(
            state.damper.inertia,
            state.damper.angular_velocity,
        )

        # Total inertial angular momentum
        hn = (h + hd).rotate(state.quaternion)
        angular_momenta.append(np.array([
            hn.x, hn.y, hn.z
        ]))
    duration_us = round(time.perf_counter() - start, 9) * 1e6
    print(f"Completed {ITERATIONS} steps in {duration_us/1e3} ms")
    print(f"({round(duration_us/ITERATIONS, 3)} us per iteration)")
    
    nose = np.array(nose)
    wingtip = np.array(wingtip)
    angular_momenta = np.array(angular_momenta)

    # Sphere (for nose vector)
    u, v = np.linspace(-np.pi/2, np.pi/2, 100), np.linspace(0, 2*np.pi, 100)
    u, v = np.meshgrid(u, v)
    x = np.cos(v) * np.cos(u)
    y = np.sin(v) * np.cos(u)
    z = np.sin(u)
    
    # Plot
    fig = plt.figure(0)
    ax = fig.add_subplot(projection="3d")
    ax.plot(nose[:, 0], nose[:, 1], nose[:, 2], color="tab:blue", label="Nose")
    ax.plot(wingtip[:, 0], wingtip[:, 1], wingtip[:, 2], color="tab:orange", label="Right Wingtip")
    ax.plot_surface(x, y, z, cmap="gray", alpha=0.2, linewidth=0)
    ax.scatter(0, 0, 0, s=50, color="red")
    ax.set_xlabel("X")
    ax.set_ylabel("Y")
    ax.set_zlabel("Z")
    plt.suptitle("Orientation")
    ax.set_xlim(-1, 1)
    ax.set_ylim(-1, 1)
    ax.set_zlim(-1, 1)
    ax.set_box_aspect([1, 1, 1])
    plt.legend()

    # Plot angular momentum components
    plt.figure(1)
    plt.plot(angular_momenta[:, 0], label="X")
    plt.plot(angular_momenta[:, 1], label="Y")
    plt.plot(angular_momenta[:, 2], label="Z")
    plt.suptitle("Angular Momentum, Inertial Frame")
    plt.legend()
    plt.xlabel("Simulation Step")
    plt.ylabel("Momentum")
    plt.show()
