// Create the canvas
var canvas = document.getElementById("game_view");
var ctx = canvas.getContext("2d");

// Background image
var bgReady = false;
var bgImage = new Image();
bgImage.onload = function () {
	bgReady = true;
};
bgImage.src = "images/background.png";

// Hero image
var heroReady = false;
var heroImage = new Image();
heroImage.onload = function () {
	heroReady = true;
};
heroImage.src = "images/hero.png";

// Monster image
var monsterReady = false;
var monsterImage = new Image();
monsterImage.onload = function () {
	monsterReady = true;
};
monsterImage.src = "images/monster.png";

// Game objects
var hero = {
	speed: 256, // movement in pixels per second
	x: canvas.width / 2,
	y: canvas.height / 2
};
var monster = {};
var monstersCaught = 0;

// Handle keyboard controls
var keysDown = {};

addEventListener("keydown", function (e) {
	keysDown[e.keyCode] = true;
}, false);

addEventListener("keyup", function (e) {
	delete keysDown[e.keyCode];
}, false);

// Initialize the game
var init = function () {
	hero.x = canvas.width / 2;
	hero.y = canvas.height / 2;

	// Throw the monster somewhere on the screen randomly
	monster.x = 32 + (Math.random() * (canvas.width - 64));
	monster.y = 32 + (Math.random() * (canvas.height - 64));
};

// Reset the game when the player catches a monster
var reset = function () {

	// Get a new monster from server
	xmlhttp = new XMLHttpRequest();
	xmlhttp.onreadystatechange = function() {
    if (this.readyState == 4 && this.status == 200) {
      var obj = JSON.parse(this.responseText);
			monster.x = obj.monster.x;
			monster.y = obj.monster.y;
    }
  };
  xmlhttp.open("GET", "http://127.0.0.1:4000/data/", true);
  xmlhttp.send();
};

// Update game objects
var update = function (modifier) {

	// Get data from server
	xmlhttp = new XMLHttpRequest();
	xmlhttp.onreadystatechange = function() {
    if (this.readyState == 4 && this.status == 200) {
      var data = JSON.parse(this.responseText);
			hero.x = data.hero.x;
			hero.y = data.hero.y;
    }
  };
  xmlhttp.open("GET", "http://127.0.0.1:4000/data/", true);
  xmlhttp.send();

	if (38 in keysDown) { // Player holding up
		var speed = {"amplitude": 0.01, "direction": eval(Math.PI/4)};
		xmlhttp = new XMLHttpRequest();
	  xmlhttp.open("POST", "http://127.0.0.1:4000/set/", true);
	  xmlhttp.send(JSON.stringify(speed));
	}
	if (40 in keysDown) { // Player holding down
		var speed = {"amplitude": 0.01, "direction": eval(Math.PI/2)};
		xmlhttp = new XMLHttpRequest();
	  xmlhttp.open("POST", "http://127.0.0.1:4000/set/", true);
	  xmlhttp.send(JSON.stringify(speed));
	}
	if (37 in keysDown) { // Player holding left
		var speed = {"amplitude": 0.01, "direction": eval(Math.PI)};
		xmlhttp = new XMLHttpRequest();
	  xmlhttp.open("POST", "http://127.0.0.1:4000/set/", true);
	  xmlhttp.send(JSON.stringify(speed));
	}
	if (39 in keysDown) { // Player holding right
		var speed = {"amplitude": 0.01, "direction": 0.0};
		xmlhttp = new XMLHttpRequest();
	  xmlhttp.open("POST", "http://127.0.0.1:4000/set/", true);
	  xmlhttp.send(JSON.stringify(speed));
	}

	// Are they touching?
	if (
		hero.x <= (monster.x + 32)
		&& monster.x <= (hero.x + 32)
		&& hero.y <= (monster.y + 32)
		&& monster.y <= (hero.y + 32)
	) {
		++monstersCaught;
		reset();
	}


	// If player touches (crosses) boundary, then flip him over to the opposite side.
	// Somewhat like what happens in "Snake" :P
	if ( hero.x <= 0 ) {
		hero.x = canvas.width - 10;
	}
	if ( hero.x >= canvas.width ) {
		hero.x = 10;
	}
	if ( hero.y <= 0 ) {
		hero.y = canvas.height - 10;
	}
	if ( hero.y >= canvas.height ) {
		hero.y = 10;
	}

};

// Draw everything
var render = function () {
	if (bgReady) {
		ctx.drawImage(bgImage, 0, 0);
	}

	if (heroReady) {
		ctx.drawImage(heroImage, hero.x, hero.y);
	}

	if (monsterReady) {
		ctx.drawImage(monsterImage, monster.x, monster.y);
	}

	// Score
	ctx.fillStyle = "rgb(250, 250, 250)";
	ctx.font = "24px Helvetica";
	ctx.textAlign = "left";
	ctx.textBaseline = "top";
	ctx.fillText("Goblins caught: " + monstersCaught, 32, 32);
};

// The main game loop
var main = function () {
	var now = Date.now();
	var delta = now - then;

	update(delta / 1000);
	render();

	then = now;

	// Request to do this again ASAP
	requestAnimationFrame(main);
};

// Cross-browser support for requestAnimationFrame
var w = window;
requestAnimationFrame = w.requestAnimationFrame || w.webkitRequestAnimationFrame || w.msRequestAnimationFrame || w.mozRequestAnimationFrame;

// Let's play this game!
var then = Date.now();
init();
main();
