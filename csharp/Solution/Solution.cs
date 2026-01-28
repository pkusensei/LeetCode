using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int PeakIndexInMountainArray(int[] arr)
    {
        int left = 0;
        int right = arr.Length - 1;
        while (left < right)
        {
            int mid = left + (right - left) / 2;
            if (arr[mid - 1] < arr[mid] && arr[mid] < arr[1 + mid]) { left = mid; }
            else if (arr[mid - 1] > arr[mid] && arr[mid] > arr[1 + mid]) { right = mid; }
            else { return mid; }
        }
        return -1;
    }
}