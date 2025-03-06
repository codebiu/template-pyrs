import com.sun.jna.Library;
import com.sun.jna.Native;
import com.sun.jna.Pointer;
import com.sun.jna.ptr.PointerByReference;

// 定义动态链接库接口
interface CalculatorLibrary extends Library {
    CalculatorLibrary INSTANCE = (CalculatorLibrary) Native.load("tempalte_lib", CalculatorLibrary.class);

    Pointer calculator_new();
    void calculator_free(Pointer ptr);
    double calculator_add(Pointer ptr, double a, double b);
    double calculator_subtract(Pointer ptr, double a, double b);
    double calculator_multiply(Pointer ptr, double a, double b);
    double calculator_divide(Pointer ptr, double a, double b, PointerByReference error);
}

public class Calculator {
    private Pointer _ptr;

    public Calculator() {
        _ptr = CalculatorLibrary.INSTANCE.calculator_new();
        if (_ptr == null) {
            throw new RuntimeException("Failed to create calculator");
        }
    }

    public void finalize() {
        if (_ptr != null) {
            CalculatorLibrary.INSTANCE.calculator_free(_ptr);
            _ptr = null;
        }
    }

    public double add(double a, double b) {
        return CalculatorLibrary.INSTANCE.calculator_add(_ptr, a, b);
    }

    public double subtract(double a, double b) {
        return CalculatorLibrary.INSTANCE.calculator_subtract(_ptr, a, b);
    }

    public double multiply(double a, double b) {
        return CalculatorLibrary.INSTANCE.calculator_multiply(_ptr, a, b);
    }

    public double divide(double a, double b) {
        PointerByReference error = new PointerByReference();
        double result = CalculatorLibrary.INSTANCE.calculator_divide(_ptr, a, b, error);
        Pointer errorPtr = error.getValue();
        if (errorPtr != null) {
            String errorMessage = errorPtr.getString(0);
            throw new ArithmeticException(errorMessage);
        }
        return result;
    }
}