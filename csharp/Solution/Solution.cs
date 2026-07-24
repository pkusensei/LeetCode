using System.Collections.Frozen;
using System.Linq.Expressions;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class SnapshotArray
{
    public SnapshotArray(int n)
    {
        Arr = [.. Enumerable.Range(0, n).Select(_ => new List<(int, int)> { (0, 0) })];
        SnapId = 0;
    }

    List<(int snap, int val)>[] Arr { get; }
    int SnapId { get; set; }

    public void Set(int index, int val)
    {
        var arr = Arr[index];
        if (arr[^1].snap == SnapId) { arr[^1] = (SnapId, val); }
        else { arr.Add((SnapId, val)); }
    }

    public int Snap()
    {
        SnapId += 1;
        return SnapId - 1;
    }

    public int Get(int index, int snap_id)
    {
        var arr = Arr[index];
        int left = 0;
        int right = arr.Count - 1;
        while (left < right)
        {
            int mid = left + (1 + right - left) / 2;
            if (arr[mid].snap > snap_id) { right = mid - 1; }
            else { left = mid; }
        }
        return arr[left].snap <= snap_id || left == 0 ? arr[left].val : arr[left - 1].val;
    }
}
