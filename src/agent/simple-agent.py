"""
A simple agent to control the hero

Author: Lennart Jern
"""

URL = '127.0.0.1:4000'

import http.client, json, time
import numpy as np
from math import sqrt

HEADERS = {"Content-type": "application/json", "Accept": "text/json"}
ACTIONS = ["Left", "Right", "Up", "Down"]

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
        # Replace the old memory
        self.memory = observation


    def make_decision(self):
        """React to an observation."""

        # Get the activity of every perceptron
        activities = [p.activity(self.memory) for p in self.perceptrons]

        index = np.argmax(activities)
        return self.perceptrons[index].action

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

def reward(data):
    """The reward is based on how close the hero is to the gnome"""
    hx = data["hero"]["x"]
    hy = data["hero"]["y"]
    gx = data["gnome"]["x"]
    gy = data["gnome"]["y"]

    distance = sqrt((gx-hx)^2 + (gy-hy)^2)
    reward = 100/(distance+100) # max 1


brain = Brain()
print("Initialized brain")

brain.observe()
brain.observe()
print(brain.memory)
print(brain.make_decision())
