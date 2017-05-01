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

class Brain:
    """
    A Brain has 4 different Perceptrons, one for each possible action.
    """

    def __init__(self):
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


    def make_decision(self):
        """React to an observation."""

        action = ""

        x_dir = self.memory[0][2] - self.memory[0][0]
        y_dir = self.memory[0][3] - self.memory[0][1]

        if (abs(x_dir) < abs(y_dir)):
            if (y_dir < 0):
                action = "Up"
            else:
                action = "Down"
        else:
            if (x_dir < 0):
                action = "Left"
            else:
                action = "Right"
        
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


brain = Brain()
print("Initialized brain")
print("Starting main loop")
while True:
    brain.observe()
    act = brain.make_decision()
    brain.execute_action(act)
    time.sleep(0.2)
