using System.Diagnostics;

Test1();
Test2();
Console.WriteLine("!!Test Passed!!");

void Test1()
{
    var n = SortedListToBST(ListNode.Make([-10, -3, 0, 5, 9]));
    var a = "[0,-3,9,-10,null,5]";
    Debug.Assert(n.ToString() == a, $"Expect: {n}\nOutput: {a}");
}

void Test2()
{
    var n = SortedListToBST(ListNode.Make([]));
    Debug.Assert(n is null);
}

TreeNode SortedListToBST(ListNode head)
{
    if (head is null) { return null; }
    if (head.next is null) { return new(head.val); }

    ListNode dummy = new(0, head);
    (var slow, var fast) = (dummy, dummy);
    var step = 0;
    while (fast.next is not null)
    {
        fast = fast.next;
        step += 1;
        if (step == 2)
        {
            slow = slow.next;
            step = 0;
        }
    }
    var temp = slow.next;
    slow.next = null;
    var root = new TreeNode()
    {
        val = temp.val,
        left = SortedListToBST(dummy.next),
        right = SortedListToBST(temp.next)
    };
    return root;
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