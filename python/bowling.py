import unittest


class Game():
    def __init__(self):
        self.pins = []

    def roll(self, pin: int):
        self.pins.append(pin)

    def score(self):
        score = 0
        current_roll = 0
        for _ in range(0, 10):
            if self.pins[current_roll] == 10:
                score += 10 + self.pins[current_roll +
                                        1] + self.pins[current_roll + 2]
                current_roll += 1
            elif self.score_for_frame(current_roll) == 10:
                score += 10 + self.pins[current_roll + 2]
                current_roll += 2
            else:
                score += self.score_for_frame(current_roll)
                current_roll += 2
        return score

    def score_for_frame(self, current_roll: int):
        return self.pins[current_roll] + self.pins[current_roll + 1]


class TestBowling(unittest.TestCase):
    def test_all_ones(self):
        game = Game()
        for _ in range(0, 20):
            game.roll(1)
        self.assertEqual(game.score(), 20)

    def test_one_spare(self):
        game = Game()
        game.roll(3)
        game.roll(7)
        game.roll(6)
        for _ in range(3, 20):
            game.roll(0)
        self.assertEqual(game.score(), 22)

    def test_one_strike(self):
        game = Game()
        game.roll(10)
        game.roll(3)
        game.roll(4)
        for _ in range(3, 19):
            game.roll(0)
        self.assertEqual(game.score(), 24)

    def test_perfect_game(self):
        game = Game()
        for _ in range(0, 12):
            game.roll(10)
        self.assertEqual(game.score(), 300)

    def test_all_gutters(self):
        game = Game()
        for _ in range(0, 20):
            game.roll(0)
        self.assertEqual(game.score(), 0)


unittest.main()
