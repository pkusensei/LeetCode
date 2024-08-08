using System.Diagnostics;

Test1();
Test2();
Console.WriteLine("!!Test Passed!!");

void Test1()
{
    var n = SortedArrayToBST([-10, -3, 0, 5, 9]);
    var a = "[0,-3,9,-10,null,5]";
    Debug.Assert(n.ToString() == a, $"Expect: {n}\nOutput: {a}");
}

void Test2()
{
    var n = SortedArrayToBST([1, 3]);
    var a = "[3,1]";
    Debug.Assert(n.ToString() == a, $"Expect: {n}\nOutput: {a}");
}

TreeNode SortedArrayToBST(int[] nums)
{
    return Solve(nums, 0, nums.Length);
}

TreeNode Solve(int[] nums, int left, int right)
{
    if (left == right) { return null; }
    if (left + 1 == right) { return new(nums[left]); }
    var mid = (right - left) / 2 + left;
    return new()
    {
        val = nums[mid],
        left = Solve(nums, left, mid),
        right = Solve(nums, mid + 1, right)
    };
}