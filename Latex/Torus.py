import torch
import torch.nn as nn
import numpy as np
import matplotlib.pyplot as plt
from mpl_toolkits.mplot3d import Axes3D

def sphere_to_torus_transformation(sphere_points, R, r, t):
    """
    Transform sphere coordinates to torus coordinates using diffusion parameter t
    """
    # Sphere coordinates (theta, phi)
    theta, phi = sphere_points[:, 0], sphere_points[:, 1]
    
    # Gradually transform from sphere to torus
    x = (R + r * torch.cos(phi) * t) * torch.cos(theta) * (1 - t) + \
        (R + r * torch.cos(phi)) * torch.cos(theta) * t
    
    y = (R + r * torch.cos(phi) * t) * torch.sin(theta) * (1 - t) + \
        (R + r * torch.cos(phi)) * torch.sin(theta) * t
    
    z = r * torch.sin(phi) * t + sphere_points[:, 2] * (1 - t)
    
    return torch.stack([x, y, z], dim=1)


class TorusDiffusionModel(nn.Module):
    def __init__(self, hidden_dim=256, time_embed_dim=128):
        super().__init__()
        
        self.time_embed = nn.Sequential(
            nn.Linear(1, time_embed_dim),
            nn.SiLU(),
            nn.Linear(time_embed_dim, time_embed_dim)
        )
        
        self.network = nn.Sequential(
            nn.Linear(3 + time_embed_dim, hidden_dim),
            nn.SiLU(),
            nn.Linear(hidden_dim, hidden_dim),
            nn.SiLU(),
            nn.Linear(hidden_dim, hidden_dim),
            nn.SiLU(),
            nn.Linear(hidden_dim, 3)  # Predicts the deformation
        )
        
    def forward(self, x, t):
        # x: sphere points [batch_size, 3]
        # t: diffusion time [batch_size, 1]
        
        t_embed = self.time_embed(t)
        x = torch.cat([x, t_embed], dim=1)
        return self.network(x)
    


class SphereToTorusDiffusion:
    def __init__(self, beta_start=1e-4, beta_end=0.02, num_timesteps=1000):
        self.num_timesteps = num_timesteps
        
        # Noise schedule
        self.betas = torch.linspace(beta_start, beta_end, num_timesteps)
        self.alphas = 1. - self.betas
        self.alpha_bars = torch.cumprod(self.alphas, dim=0)
        
    def forward_diffusion(self, x0, t):
        """Add noise to sphere points at timestep t"""
        alpha_bar = self.alpha_bars[t].view(-1, 1)
        noise = torch.randn_like(x0)
        
        x_t = torch.sqrt(alpha_bar) * x0 + torch.sqrt(1 - alpha_bar) * noise
        return x_t, noise
    
    def reverse_diffusion_step(self, model, x_t, t, guidance_weight=0.0):
        """Reverse diffusion step with optional guidance"""
        with torch.no_grad():
            # Predict noise
            predicted_noise = model(x_t, t.unsqueeze(1))
            
            # Apply guidance towards torus shape
            if guidance_weight > 0:
                torus_target = self.get_torus_shape(x_t)
                shape_guidance = torus_target - x_t
                predicted_noise = predicted_noise + guidance_weight * shape_guidance
            
            # Reverse step
            alpha = self.alphas[t]
            alpha_bar = self.alpha_bars[t]
            beta = self.betas[t]
            
            if t > 0:
                noise = torch.randn_like(x_t)
            else:
                noise = torch.zeros_like(x_t)
                
            x_prev = (1 / torch.sqrt(alpha)) * (
                x_t - (beta / torch.sqrt(1 - alpha_bar)) * predicted_noise
            ) + torch.sqrt(beta) * noise
            
            return x_prev
    
    def get_torus_shape(self, points, R=2.0, r=0.8):
        """Convert arbitrary points to torus-like structure"""
        # Project points to torus coordinates
        x, y, z = points[:, 0], points[:, 1], points[:, 2]
        
        # Convert to toroidal coordinates
        phi = torch.atan2(z, torch.sqrt(x**2 + y**2) - R)
        theta = torch.atan2(y, x)
        
        # Generate torus points
        x_torus = (R + r * torch.cos(phi)) * torch.cos(theta)
        y_torus = (R + r * torch.cos(phi)) * torch.sin(theta)
        z_torus = r * torch.sin(phi)
        
        return torch.stack([x_torus, y_torus, z_torus], dim=1)

def train_diffusion_model():
    # Generate training data - spheres of different sizes
    def generate_sphere_points(num_points, radius=1.0):
        theta = torch.rand(num_points) * 2 * np.pi
        phi = torch.acos(2 * torch.rand(num_points) - 1)
        
        x = radius * torch.sin(phi) * torch.cos(theta)
        y = radius * torch.sin(phi) * torch.sin(theta)
        z = radius * torch.cos(phi)
        
        return torch.stack([x, y, z], dim=1)
    
    # Initialize model and diffusion process
    model = TorusDiffusionModel()
    diffusion = SphereToTorusDiffusion()
    optimizer = torch.optim.Adam(model.parameters(), lr=1e-4)
    
    for epoch in range(1000):
        # Sample random sphere
        sphere_points = generate_sphere_points(1024, radius=1.0 + torch.rand(1).item())
        
        # Sample random timestep
        t = torch.randint(0, diffusion.num_timesteps, (1,)).item()
        
        # Apply forward diffusion
        noisy_points, true_noise = diffusion.forward_diffusion(sphere_points, t)
        
        # Predict noise
        predicted_noise = model(noisy_points, torch.tensor([t/1000.0]).float())
        
        # Compute loss
        loss = nn.MSELoss()(predicted_noise, true_noise)
        
        optimizer.zero_grad()
        loss.backward()
        optimizer.step()
        
        if epoch % 100 == 0:
            print(f"Epoch {epoch}, Loss: {loss.item():.4f}")
    
    return model, diffusion

def generate_torus_from_sphere(model, diffusion, num_points=1000, guidance_weight=2.0):
    """Generate torus shape starting from random sphere points"""
    
    # Start from random noise (completely noisy sphere)
    x = torch.randn(num_points, 3)
    
    # Reverse diffusion process
    for t in reversed(range(diffusion.num_timesteps)):
        x = diffusion.reverse_diffusion_step(
            model, x, torch.tensor(t), guidance_weight=guidance_weight
        )
        
        if t % 100 == 0:
            print(f"Timestep {t}")
    
    return x

# Train and generate
model, diffusion = train_diffusion_model()
torus_points = generate_torus_from_sphere(model, diffusion)

def visualize_results(sphere_points, torus_points):
    fig = plt.figure(figsize=(12, 5))
    
    # Original sphere
    ax1 = fig.add_subplot(121, projection='3d')
    ax1.scatter(sphere_points[:, 0], sphere_points[:, 1], sphere_points[:, 2], 
               alpha=0.6, s=1)
    ax1.set_title('Initial Sphere')
    ax1.set_xlim([-2, 2])
    ax1.set_ylim([-4, 4])
    ax1.set_zlim([-8, 8])
    
    # Generated torus
    ax2 = fig.add_subplot(122, projection='3d')
    ax2.scatter(torus_points[:, 0], torus_points[:, 1], torus_points[:, 2], 
               alpha=0.6, s=1)
    ax2.set_title('Generated Torus')
    ax2.set_xlim([-3, 3])
    ax2.set_ylim([-6, 6])
    ax2.set_zlim([-9, 9])
    
    plt.tight_layout()
    plt.show()

# Generate initial sphere for comparison
initial_sphere = torch.randn(1000, 3)
initial_sphere = initial_sphere / torch.norm(initial_sphere, dim=1, keepdim=True)


visualize_results(initial_sphere, torus_points)
