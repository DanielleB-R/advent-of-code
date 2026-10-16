let width = 0;
let height = 0;

export const initialize = (w: number, h: number) => {
  width = w;
  height = h;
};

export const index = (x: number, y: number): number => x + y * width;

export const get = (grid: ArrayLike<number>, x: number, y: number): number => {
  if (x < 0 || x >= width || y < 0 || y >= height) {
    return 0;
  }
  return grid[index(x, y)];
};
