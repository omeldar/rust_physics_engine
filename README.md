# Rust Physics Engine

Recently I had the idea of creating some simulations and needed some basic gravity rules for that reason. In order to cleanly seperate that I will here implement the gravity physics and use them later in the simulation project. In future if I come to add some other physics I will add them here.

## Roadmap

- [x] Define physics equations for gravitational force
- [x] (current) Define the objects to be simulated and their properties
- [ ] Create some basic simulation steps and define step length
- [ ] Visualize simulation with some library
- [ ] In each time step calculate forces acting on each object
- [ ] In each step update the positions and velocities
- [ ] Add collision detection
- [ ] Add contraints such as preventing object clipping and keep objects in certain area
- [ ] Optimize code and potentially add multithreading
- [ ] Test and validate simulation with applied physics engine

## Physics

### Gravitational Force Equation

The gravitational force between two objects can be calculated using the following equation:

$$F = G \frac{m_1 \cdot m_2}{d^2}$$

For example, if you want to calculate the gravitational force between two objects with masses of 10 kg and 20 kg, separated by a distance of 5 meters, the equation would look like this: 

$$F = 6.6743 \times 10^{-11} \frac{10 \times 20}{5^2}$$

- F is the gravitational force between the two objects, measured in Newtons (N).
- G is $6.6743 × 10^{-11}$ and is the gravitational constant, which is a fundamental constant of nature that determines the strength of the gravitational force.
- m1 is the mass of the first object.
- m2 is the mass of the second object.
- r is the distance between the two objects.

### Calculate angle of gravitational force pull

First all the single gravitational forces between the objects need to be calculated.

$$F = G \frac{m_1 m_2}{d^2}$$

Then the direction and magnitude of the gravitational force vector between the objects needs to be calculated.

$$F_x = F \cos{\theta}$$

$$F_y = F \sin{\theta}$$

- F is the magnitude of the gravitational force between the two objects.
- θ is the angle between the x-axis and the line connecting the two objects. θ can be calculated by using trigonometry.

In order to get the total force of the vector now all the other vectors need to be added up. This can be done by adding the x and y components of each force vector seperately. The resulting vector is the total force acting on the object due to the gravitational attraction of all the larger objects.

To calculate the direction and angle of the total force vector trigonometry can be used. With it we calculate the angle between the x-axis and the total force vector and this will get us the direction that the smaller object will be pulled towards.

Now depending on angle and total force the velocity and position of the pulled object needs to be updated. For that a numerical integration method, such as the Euler method or the Verlet method can be used.

The more objects are interacting and pulling on each other the more complex get the calculations. The basic approach remaints the same.

## Simulated Objects

The first thing to simulate will be some easy 2D objects with mass and some more properties attached to them.