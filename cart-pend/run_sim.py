import gymnasium as gym
import numpy as np

from cp_controller import get_action

# Setup the environment
env = gym.make("CartPole-v1", render_mode="human")
observation, info = env.reset()

# First action is random
action = env.action_space.sample()

while True:
    # Step the environment
    observation, reward, terminated, truncated, info = env.step(action)

    # Reset the environment if termination or truncation occured
    if terminated or truncated:
        observation, info = env.reset()
    else:
        # Get the action from the controller for the next step
        action = np.int64(get_action(list(observation)))
