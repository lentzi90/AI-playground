// Create the canvas
var canvas = document.getElementById("game_view");
canvas.style.backgroundColor = 'rgb(20, 100, 20)';
var ctx = canvas.getContext("2d");

// Background image
// var bgReady = false;
// var bgImage = new Image();
// bgImage.onload = function () {
// 	bgReady = true;
// };
// bgImage.src = "images/background.png";

// Hero image
var heroReady = false;
var heroImage = new Image();
heroImage.onload = function () {
	heroReady = true;
};
heroImage.src = "images/human.png";

// Monster image
var monsterReady = false;
var monsterImage = new Image();
monsterImage.onload = function () {
	monsterReady = true;
};
monsterImage.src = "images/goblin.png";

// Game objects
var hero = {};
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

var setSpeed = function(speed) {
	xmlhttp = new XMLHttpRequest();
	xmlhttp.open("POST", "http://127.0.0.1:4000/set/", true);
	xmlhttp.send(JSON.stringify(speed));
};

// Update game objects
var update = function (modifier) {

	// Amplitude of the speed vector
	var amp = 0.1;
	// Get data from server
	xmlhttp = new XMLHttpRequest();
	xmlhttp.onreadystatechange = function() {
    if (this.readyState == 4 && this.status == 200) {
      var data = JSON.parse(this.responseText);
			hero.x = data.hero.x;
			hero.y = data.hero.y;
			monster.x = data.gnome.x;
			monster.y = data.gnome.y;
			monstersCaught = data.score;
    }
  };
  xmlhttp.open("GET", "http://127.0.0.1:4000/data/", true);
  xmlhttp.send();

	if (38 in keysDown) { // Player holding up
		var speed = {"amplitude": eval(amp), "direction": eval(2 * Math.PI * 3/4)};
		setSpeed(speed);
	}
	if (40 in keysDown) { // Player holding down
		var speed = {"amplitude": eval(amp), "direction": eval(2 * Math.PI/4)};
		setSpeed(speed);
	}
	if (37 in keysDown) { // Player holding left
		var speed = {"amplitude": eval(amp), "direction": eval(2 * Math.PI/2)};
		setSpeed(speed);
	}
	if (39 in keysDown) { // Player holding right
		var speed = {"amplitude": eval(amp), "direction": eval(2 * Math.PI)};
		setSpeed(speed);
	}

};

// Draw everything
var render = function () {
	// if (bgReady) {
	// 	ctx.drawImage(bgImage, 0, 0);
	// }
	ctx.clearRect(0, 0, canvas.width, canvas.height);

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
main();
