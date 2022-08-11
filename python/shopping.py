from decimal import Decimal
from enum import Enum
import unittest


class TaxType(Enum):
    NORMAL = Decimal("0.1")
    REDUCED = Decimal("0.08")


class Item():
    def __init__(self, name: str, price_without_tax: int, tax_type: TaxType):
        self.name = name
        self.price_without_tax = price_without_tax
        self.tax_type = tax_type

    def get_price_without_tax(self):
        return self.price_without_tax

    def get_price_with_tax(self):
        return self.price_without_tax * (1 + self.tax_type.value)

    def get_name(self):
        return self.name


class Basket():
    def __init__(self, items: list[Item]):
        self.items = items

    def get_price_sum(self):
        sum = 0
        for item in self.items:
            sum += item.get_price_with_tax()
        return sum

    def add_items(self, items: list[Item]):
        self.items.extend(items)

    def get_item_names(self):
        names = []
        for item in self.items:
            names.append(item.get_name())
        return names


class TestItem(unittest.TestCase):
    def test_get_price_without_tax(self):
        item = Item("item", 100, TaxType.NORMAL)
        result = item.get_price_without_tax()
        self.assertEqual(100, result)

    def test_get_price_with_tax__normal_tax(self):
        item = Item("item", 100, TaxType.NORMAL)
        result = item.get_price_with_tax()
        self.assertEqual(110, result)

    def test_get_price_with_tax__reduced_tax(self):
        item = Item("item", 100, TaxType.REDUCED)
        result = item.get_price_with_tax()
        self.assertEqual(108, result)

    def test_get_name(self):
        name = "apple"
        item = Item(name, 100, TaxType.REDUCED)
        result = item.get_name()
        self.assertEqual(name, result)


class TestBasket(unittest.TestCase):
    def test_get_price_sum(self):
        apple = Item("apple", 100, TaxType.REDUCED)
        pencil = Item("pencil", 30, TaxType.NORMAL)
        basket = Basket([apple, apple, apple, pencil])
        result = basket.get_price_sum()
        self.assertEqual(357, result)

    def test_get_item_names(self):
        apple = Item("apple", 100, TaxType.REDUCED)
        pencil = Item("pencil", 30, TaxType.NORMAL)
        basket = Basket([apple, apple, apple, pencil])
        result = basket.get_item_names()
        self.assertEqual(["apple", "apple", "apple", "pencil"], result)

    def test_add_items(self):
        apple = Item("apple", 100, TaxType.REDUCED)
        pencil = Item("pencil", 30, TaxType.NORMAL)
        basket = Basket([apple, apple, apple, pencil])
        result = basket.get_price_sum()
        self.assertEqual(357, result)
        self.assertEqual(["apple", "apple", "apple", "pencil"],
                         basket.get_item_names())

        basket.add_items([pencil])
        result2 = basket.get_price_sum()
        self.assertEqual(390, result2)
        self.assertEqual(["apple", "apple", "apple", "pencil", "pencil"],
                         basket.get_item_names())


unittest.main()
