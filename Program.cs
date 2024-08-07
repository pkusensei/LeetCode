using System.Diagnostics;

Test1();
Test2();
Console.WriteLine("!!Test Passed!!");

void Test1()
{
    var n = BuildTree([3, 9, 20, 15, 7], [9, 3, 15, 20, 7]);
    var a = "[3,9,20,null,null,15,7]";
    Debug.Assert(n.ToString() == a, $"Expect: {n}\nOutput: {a}");
}

void Test2()
{
    var n = BuildTree([-1], [-1]);
    var a = "[-1]";
    Debug.Assert(n.ToString() == a, $"Expect: {n}\nOutput: {a}");
}

TreeNode BuildTree(int[] preorder, int[] inorder)
{
    if (preorder.Length == 0 || inorder.Length == 0) { return null; }
    var root = new TreeNode(preorder[0]);
    if (preorder.Length == 1) { return root; }

    var root_idx = Array.FindIndex(inorder, num => num == preorder[0]);

    var left_in = inorder.Take(root_idx).ToArray();
    var right_in = inorder.Skip(root_idx + 1).ToArray();
    var left_pre = preorder.Skip(1).Where(num => left_in.Contains(num)).ToArray();
    var right_pre = preorder.Skip(1).Where(num => right_in.Contains(num)).ToArray();

    root.left = BuildTree(left_pre, left_in);
    root.right = BuildTree(right_pre, right_in);

    return root;
}