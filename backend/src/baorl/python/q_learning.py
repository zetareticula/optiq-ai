import torch
import torch.nn as nn
import torch.optim as optim
import numpy as np
from collections import deque
import random

class QNetwork(nn.Module):
    def __init__(self, input_dim, output_dim):
        super(QNetwork, self).__init__()
        self.fc = nn.Sequential(
            nn.Linear(input_dim, 128),
            nn.ReLU(),
            nn.Linear(128, output_dim)
        )

    def forward(self, x):
        return self.fc(x)

def train_q_learning(plans, sla_latency_ms, sla_budget, episodes=1000, gamma=0.99, epsilon=1.0, epsilon_decay=0.995):
    env = QueryPlanEnv(plans, sla_latency_ms, sla_budget)
    q_network = QNetwork(input_dim=2, output_dim=len(plans))
    optimizer = optim.Adam(q_network.parameters(), lr=0.01)
    memory = deque(maxlen=10000)
    batch_size = 32

    for episode in range(episodes):
        state = env.reset()
        state_tensor = torch.FloatTensor(state)
        total_reward = 0

        done = False
        while not done:
            if random.random() < epsilon:
                action = env.action_space.sample()
            else:
                q_values = q_network(state_tensor)
                action = torch.argmax(q_values).item()

            next_state, reward, done, info = env.step(action)
            next_state_tensor = torch.FloatTensor(next_state)
            memory.append((state_tensor, action, reward, next_state_tensor, done))

            state_tensor = next_state_tensor
            total_reward += reward

            if len(memory) >= batch_size:
                batch = random.sample(memory, batch_size)
                states = torch.stack([t[0] for t in batch])
                actions = torch.LongTensor([t[1] for t in batch])
                rewards = torch.FloatTensor([t[2] for t in batch])
                next_states = torch.stack([t[3] for t in batch])
                dones = torch.FloatTensor([t[4] for t in batch])

                q_values = q_network(states).gather(1, actions.unsqueeze(1)).squeeze(1)
                next_q_values = q_network(next_states).max(1)[0]
                targets = rewards + gamma * next_q_values * (1 - dones)

                loss = nn.MSELoss()(q_values, targets)
                optimizer.zero_grad()
                loss.backward()
                optimizer.step()

        epsilon = max(0.1, epsilon * epsilon_decay)
        if episode % 100 == 0:
            print(f"Episode {episode}, Total Reward: {total_reward}, Plan: {info['plan_hash']}")

    return q_network