import torch
import torch.nn as nn
import torch.optim as optim

class PolicyNetwork(nn.Module):
    def __init__(self, input_dim, output_dim):
        super(PolicyNetwork, self).__init__()
        self.fc = nn.Sequential(
            nn.Linear(input_dim, 128),
            nn.ReLU(),
            nn.Linear(128, output_dim),
            nn.Softmax(dim=-1)
        )

    def forward(self, x):
        return self.fc(x)

def train_policy_gradient(plans, sla_latency_ms, sla_budget, episodes=1000):
    env = QueryPlanEnv(plans, sla_latency_ms, sla_budget)
    policy = PolicyNetwork(input_dim=2, output_dim=len(plans))
    optimizer = optim.Adam(policy.parameters(), lr=0.01)

    for episode in range(episodes):
        state = env.reset()
        state_tensor = torch.FloatTensor(state)
        probs = policy(state_tensor)
        action_dist = torch.distributions.Categorical(probs)
        action = action_dist.sample()

        next_state, reward, done, info = env.step(action.item())

        # Compute loss (REINFORCE)
        log_prob = action_dist.log_prob(action)
        loss = -log_prob * reward
        optimizer.zero_grad()
        loss.backward()
        optimizer.step()

        if episode % 100 == 0:
            print(f"Episode {episode}, Reward: {reward}, Plan: {info['plan_hash']}")

    return policy