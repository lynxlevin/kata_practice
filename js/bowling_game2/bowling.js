// this architecture needs too much conditions

module.exports = class Game {
    constructor() {
        this.frames = [];
        this.currentFrame = {rolls: 0, pins: 0};
        this.lastFrameState = '';
    }

    roll(pins) {
        this.currentFrame.rolls += 1;
        this.currentFrame.pins += pins;
        if (this.lastFrameState === 'spare') {

        }
        if (this.isSpare()) {
            this.lastFrameState = 'spare';
        } else {
            this.lastFrameState = '';
        }
        if (this.isLastRollForFrame()) {
            this.frames.push(this.currentFrame);
            this.currentFrame = {rolls: 0, pins: 0};
        }
    }

    getScore() {
        // let score = 0;
        // let rollIndex = 0;
        // for (let frame = 0; frame < 10; frame++) {
            //     if (this.isStrike(rollIndex)) {
                //         score += 10 + this.strikeBonus(rollIndex);
                //         rollIndex += 1;
                //     } else if (this.isSpare(rollIndex)) {
                    //         score += 10 + this.spareBonus(rollIndex);
                    //         rollIndex += 2;
                    //     } else {
                        //         score += this.scoreForFrame(rollIndex);
                        //         rollIndex += 2;
                        //     }
        // }
        // return score;
        let score = 0;
        this.frames.forEach(({rolls, pins}, i) => {
            score += pins;
        })
        return score;
    }

    isLastRollForFrame() {
        return this.currentFrame.rolls === 2;
    }
    isSpare() {
        return this.currentFrame.rolls === 2 && this.currentFrame.pins === 10;
    }

    // isSpare(rollIndex) {
    //     return this.rolls[rollIndex] + this.rolls[rollIndex + 1] === 10;
    // }

    // isStrike(rollIndex) {
    //     return this.rolls[rollIndex] === 10;
    // }

    // spareBonus(rollIndex) {
    //     return this.rolls[rollIndex + 2];
    // }

    // strikeBonus(rollIndex) {
    //     return this.rolls[rollIndex + 1] + this.rolls[rollIndex + 2];
    // }

    // scoreForFrame(rollIndex) {
    //     return this.rolls[rollIndex] + this.rolls[rollIndex + 1];
    // }
}
