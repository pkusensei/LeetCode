using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public Node Construct(int[][] grid)
    {
        return Construct(grid, 0, 0, grid.Length);
    }

    static Node Construct(int[][] grid, int xmin, int ymin, int n)
    {
        var origin = grid[ymin][xmin];
        if (n == 1) { return new(origin == 1, true); }

        var isLeaf = true;
        for (int y = ymin; y < ymin + n; y += 1)
        {
            for (int x = xmin; x < xmin + n; x += 1)
            {
                if (grid[y][x] != origin)
                {
                    isLeaf = false;
                    break;
                }
            }
            if (!isLeaf) { break; }
        }
        if (isLeaf) { return new(origin == 1, true); }
        var topLeft = Construct(grid, xmin, ymin, n / 2);
        var topright = Construct(grid, xmin + n / 2, ymin, n / 2);
        var bottomLeft = Construct(grid, xmin, ymin + n / 2, n / 2);
        var bottomRight = Construct(grid, xmin + n / 2, ymin + n / 2, n / 2);
        return new(origin == 1, false, topLeft, topright, bottomLeft, bottomRight);
    }
}

public class Node
{
    public bool val;
    public bool isLeaf;
    public Node topLeft;
    public Node topRight;
    public Node bottomLeft;
    public Node bottomRight;

    public Node()
    {
        val = false;
        isLeaf = false;
        topLeft = null;
        topRight = null;
        bottomLeft = null;
        bottomRight = null;
    }

    public Node(bool _val, bool _isLeaf)
    {
        val = _val;
        isLeaf = _isLeaf;
        topLeft = null;
        topRight = null;
        bottomLeft = null;
        bottomRight = null;
    }

    public Node(bool _val, bool _isLeaf, Node _topLeft, Node _topRight, Node _bottomLeft, Node _bottomRight)
    {
        val = _val;
        isLeaf = _isLeaf;
        topLeft = _topLeft;
        topRight = _topRight;
        bottomLeft = _bottomLeft;
        bottomRight = _bottomRight;
    }
}