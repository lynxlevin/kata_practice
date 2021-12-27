const Game = require('../bowling.js');

let game;

beforeEach(() => {
    game = new Game();
});

const rollMany = (n, pins) => {
    for (let i = 0; i < n; i++) {
        game.roll(pins);
    }
}

test('test gutter game', () => {
    rollMany(20, 0);
    expect(game.score()).toBe(0);
});

test('test all ones', () => {
    rollMany(20, 1);
    expect(game.score()).toBe(20);
});

test('one spare', () => {
    game.roll(2);
    game.roll(8);
    game.roll(3);
    rollMany(17, 0);
    expect(game.score()).toBe(16);
});

test('one strike', () => {
    game.roll(10);
    game.roll(5);
    game.roll(2);
    rollMany(16, 0);
    expect(game.score()).toBe(24);
});

test('perfect game', () => {
    rollMany(12, 10);
    expect(game.score()).toBe(300);
});