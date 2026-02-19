# ADCS
# Copyright (c) 2026 Joseph Hobbs

from matplotlib import pyplot as plt
import numpy as np

from adcs import AngularVelocity, Inertia, KaneDamper, Quaternion, State, Torque
from adcs import integrators as it

plt.rcParams["font.family"] = "Poppins"
plt.rcParams["font.size"]   = 12

DAMPING = 0.1

DT = 0.01
TIME = 100
ITERATIONS = int(TIME // DT)

if __name__ == "__main__":
    # Initialize state
    inertia = Inertia(1.00, 1.05, 2, 0, 0, 0)
    state = State(inertia)

    # Set damping
    state.damper = KaneDamper(
        Inertia(0.1, 0.1, 0.1, 0.0, 0.0, 0.0),
        DAMPING,    
    )
    
    # Set angular velocity
    state.angular_velocity = AngularVelocity(2, 0.3, 0.3)

    # Integrate
    fe = it.ForwardEuler(DT)
    n = Quaternion.from_vector(-1, 0, 0)
    w = Quaternion.from_vector(0, 1, 0)
    nose = []
    wingtip = []
    for _ in range(ITERATIONS):
        state = fe.step(state)
        nose.append(np.array((state.quaternion * n * state.quaternion.inv()).vector))
        wingtip.append(np.array((state.quaternion * w * state.quaternion.inv()).vector))
        print(state.angular_velocity)
    nose = np.array(nose)
    wingtip = np.array(wingtip)

    # Sphere (for nose vector)
    u, v = np.linspace(-np.pi/2, np.pi/2, 100), np.linspace(0, 2*np.pi, 100)
    u, v = np.meshgrid(u, v)
    x = np.cos(v) * np.cos(u)
    y = np.sin(v) * np.cos(u)
    z = np.sin(u)

    # Plot
    fig = plt.figure()
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
    plt.show()
