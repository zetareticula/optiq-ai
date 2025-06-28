import ray
from ray import tune
from ray.rllib.agents.ppo import PPOTrainer
from ray.rllib.agents.dqn import DQNTrainer
from environment import QueryPlanEnv

def train_baorl(plans, sla_latency_ms, sla_budget, algorithm="PPO"):
    ray.init()
    config = {
        "env": QueryPlanEnv,
        "env_config": {"plans": plans, "sla_latency_ms": sla_latency_ms, "sla_budget": sla_budget},
        "framework": "torch",
        "num_workers": 2,
    }

    trainer = PPOTrainer(config=config) if algorithm == "PPO" else DQNTrainer(config=config)
    for i in range(100):
        result = trainer.train()
        print(f"Iteration {i}, Mean Reward: {result['episode_reward_mean']}")

    return trainer