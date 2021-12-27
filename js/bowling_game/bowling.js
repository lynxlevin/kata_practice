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
        let score = 0;
        let rollIndex = 0;
        for (let frame = 0; frame < 10; frame ++) {
            if (this.rolls[rollIndex] === 10) {
                score += 10 + this.rolls[rollIndex + 1] + this.rolls[rollIndex + 2];
                rollIndex += 1;
            } else if (this.isSpare(rollIndex)) {
                score += 10 + this.rolls[rollIndex + 2];
                rollIndex += 2;
            } else {
                score += this.rolls[rollIndex] + this.rolls[rollIndex + 1];
                rollIndex += 2;
            }
        }
        return score;
    }

    isSpare(rollIndex) {
        return this.rolls[rollIndex] + this.rolls[rollIndex + 1] === 10;
    }
}

module.exports = Game;