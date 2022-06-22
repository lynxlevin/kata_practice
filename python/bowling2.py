import unittest


class Game():
    def __init__(self) -> None:
        self.rolls = []

    def roll(self, pins: int):
        self.rolls.append(pins)

    def score(self) -> int:
        return self.Score(self.rolls).score

    class Score():
        def __init__(self, rolls: list) -> None:
            self.rolls = rolls
            self.score = self.calculate()

        def calculate(self) -> int:
            score = 0
            current_roll = 0
            for _ in range(10):
                if (self.is_strike(current_roll)):
                    score += self.partial_sum_of_rolls(current_roll, 3)
                    current_roll += 1
                elif (self.is_spare(current_roll)):
                    score += self.partial_sum_of_rolls(current_roll, 3)
                    current_roll += 2
                else:
                    score += self.partial_sum_of_rolls(current_roll, 2)
                    current_roll += 2
            return score

        def partial_sum_of_rolls(self, roll_from: int, number_of_rolls: int) -> int:
            roll_to = roll_from + number_of_rolls
            slice = self.rolls[roll_from:roll_to]
            return sum(slice)

        def is_strike(self, current_roll: int) -> bool:
            return self.partial_sum_of_rolls(current_roll, 1) == 10

        def is_spare(self, current_roll: int) -> bool:
            return self.partial_sum_of_rolls(current_roll, 2) == 10


class TestBowling(unittest.TestCase):
    @classmethod
    def setUp(self):
        self.game = Game()

    @classmethod
    def roll_many(self, pins: int, rolls: int):
        for _ in range(0, rolls):
            self.game.roll(pins)

    def test_all_gutters(self):
        self.roll_many(0, 20)
        self.assertEqual(self.game.score(), 0)

    def test_all_ones(self):
        self.roll_many(1, 20)
        self.assertEqual(self.game.score(), 20)

    def test_all_twos(self):
        self.roll_many(2, 20)
        self.assertEqual(self.game.score(), 40)

    def test_one_spare(self):
        self.game.roll(4)
        self.game.roll(6)
        self.game.roll(3)
        self.roll_many(0, 17)
        self.assertEqual(self.game.score(), 16)

    def test_one_strike(self):
        self.game.roll(10)
        self.game.roll(2)
        self.game.roll(5)
        self.roll_many(0, 16)
        self.assertEqual(self.game.score(), 24)

    def test_perfect_game(self):
        self.roll_many(10, 12)
        self.assertEqual(self.game.score(), 300)


unittest.main()
