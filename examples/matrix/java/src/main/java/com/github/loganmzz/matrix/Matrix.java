package com.github.loganmzz.matrix;

import java.util.Arrays;
import java.util.concurrent.ForkJoinPool;
import java.util.concurrent.RecursiveAction;

import static java.lang.String.format;

public class Matrix {

	private static final int MUL_PAR_THREHOLD = 32768;

	private int[] data;
	private int cols;
	private int rows;

	public Matrix(int cols, int rows) {
		this.data = new int[cols * rows];
		this.cols = cols;
		this.rows = rows;
	}

	public static Matrix identity(int size) {
		Matrix m = new Matrix(size, size);
		for (int i = 0; i < size; i++) {
			m.set(i, i, 1);
		}
		return m;
	}

	protected int index(int col, int row) {
		if (col >= cols || row >= rows) {
			throw new IllegalArgumentException(format("Invalid index (%d, %d)", col, row));
		}
		return col + cols * row;
	}

	protected int[] coord(int index) {
		if (index >= data.length) {
			throw new IllegalArgumentException(format("Invalid index '%d'", index));
		}
		int[] coord = { index % cols, index / cols };
		return coord;
	}

	public Matrix set(int col, int row, int val) {
		data[index(col, row)] = val;
		return this;
	}

	public int get(int col, int row) {
		return data[index(col, row)];
	}

	protected void setForMul(Matrix lhs, Matrix rhs, int index) {
		int[] coord = coord(index);
		int col = coord[0];
		int row = coord[1];
		int val = 0;
		for (int i = 0; i < lhs.cols; i++) {
			val += lhs.get(i, row) * rhs.get(col, i);
		}
		data[index] = val;
	}

	protected static Matrix initMatrixForMul(Matrix lhs, Matrix rhs) {
		if (lhs.cols != rhs.rows) {
			throw new IllegalArgumentException(format("Incompatible matrices (%d,%d)x(%d,%d)", lhs.cols, lhs.rows, rhs.cols, rhs.rows));
		}
		return new Matrix(rhs.cols, lhs.rows);
	}

	public Matrix mulSeq(Matrix rhs) {
		Matrix r = initMatrixForMul(this, rhs);
		for (int i = 0; i < r.data.length; i++) {
			r.setForMul(this, rhs, i);
		}
		return r;
	}

	class MulParRecAction extends RecursiveAction {
		Matrix lhs; Matrix rhs; int index; int length;

		MulParRecAction(Matrix lhs, Matrix rhs, int index, int length) {
			this.lhs = lhs;
			this.rhs = rhs;
			this.index = index;
			this.length = length;
		}

		MulParRecAction(Matrix lhs, Matrix rhs) {
			this(lhs, rhs, 0, data.length);
		}


		@Override
		protected void compute() {
			if (length <= MUL_PAR_THREHOLD) {
				int bound = index + length;
				for (int i = index; i < bound; i++) {
					setForMul(lhs, rhs, i);
				}
			} else {
				int split = length / 2;
				invokeAll(
						new MulParRecAction(lhs, rhs, index + 0    , split  - 0),
						new MulParRecAction(lhs, rhs, index + split, length - split)
				);
			}
		}
	}

	public Matrix mulPar(Matrix rhs) {
		Matrix r = initMatrixForMul(this, rhs);
		ForkJoinPool.commonPool().invoke(r.new MulParRecAction(this, rhs));
		return r;
	}

	@Override
	public int hashCode() {
		return Arrays.hashCode(data);
	}

	@Override
	public boolean equals(Object obj) {
		if (obj == this) {
			return true;
		}
		if (!(obj instanceof Matrix)) {
			return false;
		}
		Matrix that = (Matrix) obj;
		return
				this.cols == that.cols && this.rows == that.rows && Arrays.equals(this.data, that.data);
	}

	@Override
	public String toString() {
		return format("Matrix(%dx%d) %s", cols, rows, Arrays.toString(data));
	}
}