import gym
import numpy as np
from gym import spaces

class QueryPlanEnv(gym.Env):
    def __init__(self, plans, sla_latency_ms, sla_budget):
        super(QueryPlanEnv, self).__init__()
        self.plans = plans  # List of (plan_hash, cost, latency) tuples
        self.sla_latency_ms = sla_latency_ms
        self.sla_budget = sla_budget
        self.current_plan_idx = 0

        # Action space: select a plan (discrete)
        self.action_space = spaces.Discrete(len(plans))
        # Observation space: plan features (cost, latency)
        self.observation_space = spaces.Box(
            low=np.array([0.0, 0.0]), high=np.array([float('inf'), float('inf')]), dtype=np.float32
        )

    def reset(self):
        self.current_plan_idx = 0
        return np.array([self.plans[0][1], self.plans[0][2]], dtype=np.float32)

    def step(self, action):
        # Select plan
        selected_plan = self.plans[action]
        cost, latency = selected_plan[1], selected_plan[2]

        # Reward: negative latency if within SLA, penalize if exceeding budget
        reward = -latency if latency <= self.sla_latency_ms and cost <= self.sla_budget else -1000.0
        done = True  # Single-step environment for simplicity
        info = {"plan_hash": selected_plan[0]}

        return (
            np.array([cost, latency], dtype=np.float32),
            reward,
            done,
            info
        )