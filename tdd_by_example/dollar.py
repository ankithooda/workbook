class Dollar():

    def __init__(self, amount):
        self.amount = amount

    def times(self, n):
        return Dollar(self.amount * n)
