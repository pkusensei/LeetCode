using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public ListNode AddTwoNumbers(ListNode l1, ListNode l2)
    {
        var (s1, s2) = (AsStack(l1), AsStack(l2));
        var s = new Stack<int>();
        var carry = 0;
        while (s1.Count > 0 && s2.Count > 0)
        {
            var n1 = s1.Pop();
            var n2 = s2.Pop();
            var sum = n1 + n2 + carry;
            s.Push(sum % 10);
            carry = sum / 10;
        }
        while (s1.TryPop(out int n1))
        {
            var sum = n1 + carry;
            s.Push(sum % 10);
            carry = sum / 10;
        }
        while (s2.TryPop(out int n2))
        {
            var sum = n2 + carry;
            s.Push(sum % 10);
            carry = sum / 10;
        }
        if (carry > 0) { s.Push(carry); }

        ListNode dummy = new(0);
        var curr = dummy;
        while (s.TryPop(out int n))
        {
            ListNode node = new(n);
            curr.next = node;
            curr = curr.next;
        }
        return dummy.next;
    }

    static Stack<int> AsStack(ListNode node)
    {
        Stack<int> res = [];
        var curr = node;
        while (curr is not null)
        {
            res.Push(curr.val);
            curr = curr.next;
        }
        return res;
    }
}
