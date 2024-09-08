using System.Diagnostics;
using LList;
using Tree;

Test1();
Test2();
Console.WriteLine("!!Test Passed!!");

void Test1()
{
    var a = ListNode.Make([1, 2, 3]);
    var b = SplitListToParts(a, 5);
    var c = "[[1],[2],[3],[],[]]";
    Debug.Assert(b.Print() == c, $"Output: {b.Print()}\nExpect: {c}");
}

void Test2()
{
    var a = ListNode.Make([1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    var b = SplitListToParts(a, 3);
    var c = "[[1,2,3,4],[5,6,7],[8,9,10]]";
    Debug.Assert(b.Print() == c, $"Output: {b.Print()}\nExpect: {c}");
}

ListNode[] SplitListToParts(ListNode head, int k)
{
    var count = 0;
    var curr = head;
    while (curr is not null)
    {
        curr = curr.next;
        count += 1;
    }
    var div = count / k;
    var rem = count % k;
    var nums = new int[k];
    Array.Fill(nums, div);
    for (int i = 0; i < rem; i++)
    {
        nums[i] += 1;
    }
    var res = new ListNode[k];
    Array.Fill(res, null);
    curr = head;
    for (int i = 0; i < k; i++)
    {
        (var a, var b) = Take(curr, nums[i]);
        res[i] = a;
        curr = b;
    }
    return res;
}

(ListNode, ListNode) Take(ListNode head, int n)
{
    if (head is null) { return (null, null); }
    var prev = head;
    var curr = head;
    for (int i = 0; i < n; i++)
    {
        prev = curr;
        curr = curr.next;
    }
    prev.next = null;
    return (head, curr);
}
