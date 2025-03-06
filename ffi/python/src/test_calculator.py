import unittest
from calculator import Calculator

class TestCalculator(unittest.TestCase):
    def setUp(self):
        self.calc = Calculator()

    def tearDown(self):
        del self.calc

    def test_add(self):
        self.assertEqual(self.calc.add(2.0, 3.0), 5.0)

    def test_subtract(self):
        self.assertEqual(self.calc.subtract(5.0, 3.0), 2.0)

    def test_multiply(self):
        self.assertEqual(self.calc.multiply(4.0, 2.0), 8.0)

    def test_divide(self):
        self.assertEqual(self.calc.divide(6.0, 2.0), 3.0)
        with self.assertRaises(ZeroDivisionError):
            self.calc.divide(1.0, 0.0)

if __name__ == '__main__':
    unittest.main()