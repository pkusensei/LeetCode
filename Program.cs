using System.Diagnostics;

Test1();
Test2();
Console.WriteLine("!!Test Passed!!");

void Test1()
{
    var n = BuildTree([9, 3, 15, 20, 7], [9, 15, 7, 20, 3]);
    var a = "[3,9,20,null,null,15,7]";
    Debug.Assert(n.ToString() == a, $"Expect: {n}\nOutput: {a}");
}

void Test2()
{
    var n = BuildTree([-1], [-1]);
    var a = "[-1]";
    Debug.Assert(n.ToString() == a, $"Expect: {n}\nOutput: {a}");
}

TreeNode BuildTree(int[] inorder, int[] postorder)
{
    if (postorder.Length == 0 || inorder.Length == 0) { return null; }
    var root = new TreeNode(postorder.Last());
    if (postorder.Length == 1) { return root; }

    var root_idx = Array.FindIndex(inorder, num => num == root.val);

    var left_in = inorder.Take(root_idx).ToArray();
    var right_in = inorder.Skip(root_idx + 1).ToArray();
    var left_post = postorder.SkipLast(1).Where(num => left_in.Contains(num)).ToArray();
    var right_post = postorder.SkipLast(1).Where(num => right_in.Contains(num)).ToArray();

    root.left = BuildTree(left_in, left_post);
    root.right = BuildTree(right_in, right_post);

    return root;
}