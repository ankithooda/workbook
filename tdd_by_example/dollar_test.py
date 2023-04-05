from dollar import Dollar

def test_multiplication():
    
    money = Dollar(5)
    assert money.times(2) == 10

def test_multiple_multiplications():

    money = Dollar(5)

    assert money.times(2) == 10
    assert money.times(3) == 15
