using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public Node Intersect(Node node1, Node node2)
    {
        switch (node1.isLeaf, node2.isLeaf)
        {
            case (true, true):
                return new() { val = node1.val || node2.val, isLeaf = true };
            case (false, false):
                var top_left = Intersect(node1.topLeft, node2.topLeft);
                var top_right = Intersect(node1.topRight, node2.topRight);
                var bot_left = Intersect(node1.bottomLeft, node2.bottomLeft);
                var bot_right = Intersect(node1.bottomRight, node2.bottomRight);
                if (top_left.isLeaf && top_right.isLeaf && bot_left.isLeaf && bot_right.isLeaf
                && top_left.val == top_right.val && top_left.val == bot_left.val && top_left.val == bot_right.val)
                {
                    return new() { val = top_left.val, isLeaf = true };
                }
                return new(false, false, top_left, top_right, bot_left, bot_right);
            case (true, false): return Merge(node2, node1.val);
            default: return Merge(node1, node2.val);
        }

        static Node Merge(Node node, bool val)
        {
            if (node.isLeaf || val)
            {
                return new() { val = node.val || val, isLeaf = true };
            }
            var top_left = Merge(node.topLeft, val);
            var top_right = Merge(node.topRight, val);
            var bot_left = Merge(node.bottomLeft, val);
            var bot_right = Merge(node.bottomRight, val);
            if (top_left.isLeaf && top_right.isLeaf && bot_left.isLeaf && bot_right.isLeaf
            && top_left.val == top_right.val && top_left.val == bot_left.val && top_left.val == bot_right.val)
            {
                return new() { val = top_left.val, isLeaf = true };
            }
            return new(false, false, top_left, top_right, bot_left, bot_right);
        }
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

    public Node() { }
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