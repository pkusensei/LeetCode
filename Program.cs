using System.Collections;
using System.Diagnostics;
using System.Text;

Test1();
Test2();
Console.WriteLine("!!Test Passed!!");

void Test1()
{
    ListNode n = RotateRight(ListNode.Make([1, 2, 3, 4, 5]), 2);
    var a = "[4,5,1,2,3]";
    Debug.Assert(n.ToString() == a, $"Output: {n}\nExpect: {a}");
}

void Test2()
{
    ListNode n = RotateRight(ListNode.Make([0, 1, 2]), 4);
    var a = "[2,0,1]";
    Debug.Assert(n.ToString() == a, $"Output: {n}\nExpect: {a}");
}


ListNode RotateRight(ListNode head, int k)
{
    ListNode dummy = new(0, head);
    var length = Length(dummy);
    if (length < 2)
    {
        return head;
    }

    k %= length;
    for (int i = 0; i < k; i++)
    {
        var tail = dummy.next;
        var prev = dummy;
        while (tail.next is not null)
        {
            // 1 -> 2
            prev = tail;
            tail = tail.next;
        }
        prev.next = null;
        tail.next = dummy.next;
        dummy.next = tail;
    }

    return dummy.next;
}

int Length(ListNode dummy)
{
    var count = 0;
    while (dummy.next is not null)
    {
        dummy = dummy.next;
        count += 1;
    }
    return count;
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
