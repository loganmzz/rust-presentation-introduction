package com.github.loganmzz.matrix;

import org.junit.Rule;
import org.junit.Test;
import org.junit.rules.ErrorCollector;

import java.util.Arrays;
import java.util.function.BiFunction;

import static java.lang.String.format;
import static org.hamcrest.CoreMatchers.equalTo;
import static org.junit.Assert.assertEquals;

public class MatrixTest {

	@Rule
	public ErrorCollector collector = new ErrorCollector();

	static Matrix loadMatrix(int size) {
		return loadMatrix(size, size);
	}

	static Matrix loadMatrix(int cols, int rows) {
		Matrix m = new Matrix(cols, rows);
		int index = 0;
		for (int col = 0; col < cols; col++) {
			for (int row = 0; row < rows; row++) {
				m.set(col, row, index++);
			}
		}
		return m;
	}

	static Matrix square(int... values) {
		if (values.length != 4) {
			throw new IllegalArgumentException(format("%s doesn't contain exactly 4 values", Arrays.toString(values)));
		}
		Matrix m = new Matrix(2, 2);
		m.set(0, 0, values[0]);
		m.set(1, 0, values[1]);
		m.set(0, 1, values[2]);
		m.set(1, 1, values[3]);
		return m;
	}

	@Test
	public void defaultMatrixShouldBeFullOfZeroes() {
		Matrix m = new Matrix(4096, 4096);
		for (int col = 0; col < 4096; col++) {
			for (int row = 0; row < 4096; row++) {
				collector.checkThat(format("(%d,%d)", col, row), m.get(col, row), equalTo(0));
			}
		}
	}

	@Test
	public void identifyMatrixShouldBeFullOfZeroesButOneInDiagonal() {
		Matrix m = Matrix.identity(4096);
		for (int col = 0; col < 4096; col++) {
			for (int row = 0; row < 4096; row++) {
				collector.checkThat(format("(%d,%d)", col, row), m.get(col, row), equalTo(col != row ? 0 : 1));
			}
		}
	}

	@Test
	public void mulAndMulParShouldGiveBothEquivalentMatrices() {
		Matrix m = loadMatrix(64);

		Matrix mul = m.mulSeq(m);
		Matrix mulPar = m.mulPar(m);

		assertEquals(mul, mulPar);
	}


}
