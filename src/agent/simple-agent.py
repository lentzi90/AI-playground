"""
A simple agent to control the hero

Author: Lennart Jern
"""

URL = '127.0.0.1:4000'

import http.client, json, time
import numpy as np
from math import sqrt, pi
from random import random, randrange

HEADERS = {"Content-type": "application/json", "Accept": "text/json"}
ACTIONS = ["Left", "Right", "Up", "Down"]
DIRECTIONS = {"Left": 2*pi/2, "Right": 2*pi, "Up": 2*pi*3/4, "Down": 2*pi/4}

class UnexpectedResponse(Exception): pass

class Perceptron:
    """
    A Perceptron or neuron that reacts with a certain activity level depending on input
    """

    def __init__(self, action):
        self.action = action
        self.weights = np.ones(shape=(2,4))

    def activity(self, observation):
        """Calculate the activity based on the given image."""
        result = np.sum(np.multiply(observation, self.weights))
        act = 1/(1+np.exp(-result))
        return act

class Brain:
    """
    A Brain has 4 different Perceptrons, one for each possible action.
    """

    def __init__(self):
        self.perceptrons = [Perceptron(a) for a in ACTIONS]
        self.memory = np.ones(shape=(2,4))
        self.last_action = ACTIONS[0]
        self.last_activities = np.zeros(4)
        self.last_reward = 0
        self.old_memory = np.ones(shape=(2,4))
        self.learning_rate = 0.1

    def observe(self):
        """
        Make an observation of the world and add it to the memory.
        """

        # Initialize the observation
        observation = np.ones(shape=(2,4))
        # Get new data
        data = get_data()
        # Extract the data
        observation[0][0] = data["hero"]["x"]
        observation[0][1] = data["hero"]["y"]
        observation[0][2] = data["gnome"]["x"]
        observation[0][3] = data["gnome"]["y"]
        # Add the memories
        observation[1] = self.memory[0]
        # Update memory
        self.old_memory = self.memory
        self.memory = observation

        # Get reward and update Perceptrons
        reward = get_reward(data)
        self.teach_perceptrons(reward)


    def make_decision(self):
        """React to an observation."""

        # Execute random actions every now and then
        if random() > 0.87:
            index = randrange(0, 4)
            return self.perceptrons[index].action

        # Get the activity of every perceptron
        activities = [p.activity(self.memory) for p in self.perceptrons]

        # Add activity levels to memory
        self.last_activities = activities
        # Find the max level
        index = np.argmax(activities)
        action = self.perceptrons[index].action
        self.last_action = action
        return action

    def execute_action(self, action):
        """Send commands to the server."""
        amplitude = 0.1
        direction = DIRECTIONS[action]

        server = http.client.HTTPConnection(URL)
        params = json.dumps({'amplitude': amplitude, 'direction': direction})
        server.request('POST', '/set', params, HEADERS)
        response = server.getresponse()
        status = response.status
        #response.close()
        if (status >= 200) and (status < 300):
            return response
        else:
            raise UnexpectedResponse(response)

    def teach_perceptrons(self, reward):
        """Update the weights based on the reward."""
        performance = reward - self.last_reward
        responsible = next(p for p in self.perceptrons if p.action == self.last_action)
        delta = performance*self.old_memory
        responsible.weights = responsible.weights + delta*self.learning_rate
        responsible.weights = np.minimum(responsible.weights, 1)

        if (performance < 0):
            for p in self.perceptrons:
                if p.action != self.last_action:
                    p.weights = p.weights - delta*self.learning_rate
                    p.weights = np.minimum(p.weights, 1)

        # Update last reward
        self.last_reward = reward

def get_data():
    """Reads the current state of the world"""
    server = http.client.HTTPConnection(URL)
    server.request('GET','/data')
    response = server.getresponse()
    if (response.status == 200):
        data = response.read()
        response.close()
        return json.loads(data.decode())
    else:
        return UnexpectedResponse(response)

def get_reward(data):
    """The reward is based on how close the hero is to the gnome"""
    hx = data["hero"]["x"]
    hy = data["hero"]["y"]
    gx = data["gnome"]["x"]
    gy = data["gnome"]["y"]

    distance = sqrt((gx-hx)**2 + (gy-hy)**2)
    reward = 100/(distance+100) # max 1
    return reward


brain = Brain()
print("Initialized brain")
print("Starting main loop")
while True:
    brain.observe()
    act = brain.make_decision()
    brain.execute_action(act)
    time.sleep(0.2)
    print(brain.perceptrons[0].weights)
