class Game {
    constructor() {
        this.rolls = Array().fill(0, 0, 21);
        this.currentRoll = 0;
    }

    roll(pins) {
        this.rolls[this.currentRoll] = pins;
        this.currentRoll += 1;
    }

    score() {
        return this.rolls.reduce((sum, pins) => { return sum + pins; });
    }
}

module.exports = Game;