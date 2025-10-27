using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public ListNode[] SplitListToParts(ListNode head, int k)
    {
        ListNode[] res = new ListNode[k];
        if (head is null) { return res; }
        int len = 0;
        var curr = head;
        while (curr is not null)
        {
            len += 1;
            curr = curr.next;
        }
        int ave = len / k;
        int rem = len % k;
        curr = head;
        for (int i = 0; i < k; i++)
        {
            int count = ave + (rem > 0 ? 1 : 0);
            rem -= 1;
            if (count == 0) { break; }
            while (count > 1)
            {
                curr = curr.next;
                count -= 1;
            }
            res[i] = head;
            head = curr.next;
            curr.next = null;
            curr = head;
        }
        return res;
    }
}