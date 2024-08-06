using System.Collections;
using System.Diagnostics;
using System.Text;

Test1();
Test2();
Console.WriteLine("!!Test Passed!!");

void Test1()
{
    ListNode n = ReverseBetween(ListNode.Make([1, 2, 3, 4, 5]), 2, 4);
    var a = "[1,4,3,2,5]";
    Debug.Assert(n.ToString() == a, $"Output: {n}\nExpect: {a}");
}

void Test2()
{
    ListNode n = ReverseBetween(ListNode.Make([5]), 1, 1);
    var a = "[5]";
    Debug.Assert(n.ToString() == a, $"Output: {n}\nExpect: {a}");
}


ListNode ReverseBetween(ListNode head, int left, int right)
{
        if (head is null || left == right) { return head; }

        ListNode dummy = new(0, head);
        var prev = dummy;
        for (int i = 0; i < left - 1; i++)
        {
            prev = prev.next;
        }
        var curr = prev.next;
        for (int i = 0; i < right - left; i++)
        {
            var temp = curr.next;
            curr.next = temp.next;
            temp.next = prev.next;
            prev.next = temp;
        }
        return dummy.next;
}

class ListNode : IEnumerable<int>
{
    public int val;
    public ListNode next;
    public ListNode(int val = 0, ListNode next = null)
    {
        this.val = val;
        this.next = next;
    }

    public static ListNode Make(IEnumerable<int> nums)
    {
        ListNode dummy = new(0, null);
        var curr = dummy;
        foreach (var num in nums)
        {
            var temp = new ListNode(num, null);
            curr.next = temp;
            curr = curr.next;
        }
        return dummy.next;
    }

    public IEnumerator<int> GetEnumerator()
    {
        var curr = this;
        while (curr is not null)
        {
            yield return curr.val;
            curr = curr.next;
        }
    }

    public override string ToString()
    {
        var curr = this;
        var sb = new StringBuilder("[");
        while (curr is ListNode n)
        {
            sb.AppendFormat("{0},", n.val);
            curr = n.next;
        }
        sb.Remove(sb.Length - 1, 1);
        sb.Append(']');
        return sb.ToString();
    }

    IEnumerator IEnumerable.GetEnumerator()
    {
        return GetEnumerator();
    }
}
