package com.github.loganmzz.matrix;

import org.junit.Test;
import org.junit.runner.RunWith;
import org.junit.runners.Parameterized;

import java.util.Collection;
import java.util.function.BiFunction;

import static com.github.loganmzz.matrix.MatrixTest.loadMatrix;
import static com.github.loganmzz.matrix.MatrixTest.square;
import static java.util.Arrays.asList;
import static org.junit.Assert.assertEquals;

@RunWith(Parameterized.class)
public class MatrixMulTest {

	@Parameterized.Parameters(name = "{0}")
	public static Collection<Object[]> cases() {
		return asList(new Object[][] {
				newCase("Sequential", Matrix::mulSeq),
				newCase("Parallel", Matrix::mulPar)
		});
	}

	static Object[] newCase(String name, BiFunction<Matrix, Matrix, Matrix> fn) {
		return new Object[] { name, fn };
	}

	@Parameterized.Parameter(0)
	public String name;

	@Parameterized.Parameter(1)
	public BiFunction<Matrix, Matrix, Matrix> fn;

	@Test
	public void mulTwoSquaresShouldGiveCorrectValue() {
		assertEquals(square(19, 22, 43, 50), fn.apply(square(1, 2, 3, 4), square(5, 6, 7, 8)));
	}

	@Test
	public void mulByIdentityShouldGiveEquivalentMatrix() {
		Matrix m = loadMatrix(64);
		Matrix i = Matrix.identity(64);
		assertEquals(m, fn.apply(m, i));
	}

	@Test
	public void mulFromIdentityShouldGiveEquivalentMatrix() {
		Matrix m = loadMatrix(64);
		Matrix i = Matrix.identity(64);
		assertEquals(m, fn.apply(i, m));
	}
}
