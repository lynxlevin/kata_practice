class Item
    def initialize(name, price, tax_rate)
        @name = name
        @price = price
        @tax_rate = tax_rate
    end

    def price_with_tax
        return @price * @tax_rate
    end
end

class Basket
    def initialize(items)
        @items = items
    end

    def sum
        sum = 0
        @items.each do |item|
            sum += item[0].price_with_tax() * item[1]
        end
        puts sum
    end
end

apple = Item.new('apple', 100, 1.08)
soap = Item.new('soap', 300, 1.1)
wine = Item.new('wine', 5800, 1.1)
basket = Basket.new([[apple, 2], [soap, 5], [wine, 8]])
basket.sum

