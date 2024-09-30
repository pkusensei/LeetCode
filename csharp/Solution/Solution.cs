using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public Node Intersect(Node quadTree1, Node quadTree2)
    {
        if (quadTree1.isLeaf && quadTree2.isLeaf)
        {
            return new(quadTree1.val || quadTree2.val, true);
        }
        Node tl = Intersect(quadTree1.topLeft ?? new(quadTree1.val, quadTree1.isLeaf), quadTree2.topLeft ?? new(quadTree2.val, quadTree2.isLeaf));
        Node tr = Intersect(quadTree1.topRight ?? new(quadTree1.val, quadTree1.isLeaf), quadTree2.topRight ?? new(quadTree2.val, quadTree2.isLeaf));
        Node bl = Intersect(quadTree1.bottomLeft ?? new(quadTree1.val, quadTree1.isLeaf), quadTree2.bottomLeft ?? new(quadTree2.val, quadTree2.isLeaf));
        Node br = Intersect(quadTree1.bottomRight ?? new(quadTree1.val, quadTree1.isLeaf), quadTree2.bottomRight ?? new(quadTree2.val, quadTree2.isLeaf));
        if (tl.isLeaf && tr.isLeaf && bl.isLeaf && br.isLeaf && tl.val == tr.val && tr.val == bl.val && bl.val == br.val)
        {
            return new(tl.val, true);
        }
        else
        {
            return new(false, false, tl, tr, bl, br);
        }
    }

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