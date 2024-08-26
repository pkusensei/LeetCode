using System.Diagnostics;

Test1();
Test2();
Console.WriteLine("!!Test Passed!!");

void Test1()
{
}

void Test2()
{
}

IList<int> Postorder(Node root)
{
    var res = new List<int>();
    if (root is null) { return res; }

    var stack = new Stack<Node>();
    stack.Push(root);
    while (stack.TryPop(out var curr))
    {
        res.Add(curr.val);
        foreach (var item in curr.children)
        {
            stack.Push(item);
        }
    }
    res.Reverse();
    return res;
}

IList<int> Postorder2(Node root)
{
    var res = new List<int>();
    if (root is null) { return res; }

    var nodeStack = new Stack<Node>();
    var revStack = new Stack<Node>();
    nodeStack.Push(root);
    while (nodeStack.TryPop(out var curr))
    {
        revStack.Push(curr);
        foreach (var item in curr.children)
        {
            nodeStack.Push(item);
        }
    }
    while (revStack.TryPop(out var curr))
    {
        res.Add(curr.val);
    }
    return res;
}

//   1
// 2 3 4

// 1
// 2 3 4

// 1 4 3 2
// 2 3 4 1

//  1
// 2 3

// 1
// 2 3

// 1 3 2
// 2 3 1