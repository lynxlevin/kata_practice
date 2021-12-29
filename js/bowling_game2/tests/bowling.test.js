const Game = require('../bowling.js');

let game;

beforeEach(() => {
    game = new Game();
})

const rollMany = (n, pins) => {
    for (let i = 0; i < n; i++) {
        game.roll(pins);
    }
}

test('test all ones', () => {
    rollMany(20, 1);
    expect(game.getScore()).toBe(20);
})

test('test gutter game', () => {
    rollMany(20, 0);
    expect(game.getScore()).toBe(0);
});

test('one spare', () => {
    game.roll(3);
    game.roll(7);
    game.roll(8);
    rollMany(17, 0);
    expect(game.getScore()).toBe(26);
})

test('one strike', () => {
    game.roll(10);
    game.roll(2);
    game.roll(3);
    rollMany(16, 0);
    expect(game.getScore()).toBe(20);
})

test('perfect game', () => {
    rollMany(12, 10);
    expect(game.getScore()).toBe(300);
})