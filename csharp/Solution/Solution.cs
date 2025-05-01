using System.Buffers;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int MaxTaskAssign(int[] tasks, int[] workers, int pills, int strength)
    {
        Array.Sort(tasks);
        Array.Sort(workers, (a, b) => b.CompareTo(a));
        int left = 0;
        int right = tasks.Length;
        while (left < right)
        {
            int mid = left + (right + 1 - left) / 2;
            if (Check(mid, pills)) { left = mid; }
            else { right = mid - 1; }
        }
        return left;

        bool Check(int mid, int pills)
        {
            if (mid > workers.Length) { return false; }
            int taski = 0;
            List<int> deque = new(mid);
            // worker[i] goes from mid-1 to 0; least to most
            for (int i = mid - 1; i >= 0; i -= 1)
            {
                if (deque.Count == 0 && taski < mid)
                {
                    deque.Insert(0, tasks[taski]);
                    taski += 1;
                }
                if (deque.Last() <= workers[i])
                {
                    deque.RemoveAt(deque.Count - 1); // worker[i] finished last in deque
                }
                else
                {
                    if (pills == 0) { return false; }
                    if (deque.Last() > workers[i] + strength) { return false; }
                    // Add all thats possible with worker[i] and pill
                    while (taski < mid && tasks[taski] <= workers[i] + strength)
                    {
                        deque.Insert(0, tasks[taski]);
                        taski += 1;
                    }
                    // worker[i] finishes first in queue with pill
                    deque.RemoveAt(0);
                    pills -= 1;
                }
            }
            return true;
        }
    }
}

