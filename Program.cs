using System.Diagnostics;
using System.Text;

Test1();
Test2();
Console.WriteLine("!!Test Passed!!");

void Test1()
{
    var node = TreeNode.Make([7, 3, 15, null, null, 9, 20]);
    var it = new BSTIterator(node);
    Debug.Assert(it.Next() == 3);    // return 3
    Debug.Assert(it.Next() == 7);    // return 7
    Debug.Assert(it.HasNext()); // return True
    Debug.Assert(it.Next() == 9);    // return 9
    Debug.Assert(it.HasNext()); // return True
    Debug.Assert(it.Next() == 15);    // return 15
    Debug.Assert(it.HasNext()); // return True
    Debug.Assert(it.Next() == 20);    // return 20
    Debug.Assert(!it.HasNext()); // return False
}

void Test2()
{
}

public class BSTIterator
{
    public Stack<TreeNode> Nodes { get; }

    public BSTIterator(TreeNode root)
    {
        Nodes = [];
        Push(root);
    }

    public int Next()
    {
        var node = Nodes.Pop();
        Push(node.right);
        return node.val;
    }

    public bool HasNext()
    {
        return Nodes.Count > 0;
    }

    void Push(TreeNode node)
    {
        while (node is not null)
        {
            Nodes.Push(node);
            node = node.left;
        }
    }
}