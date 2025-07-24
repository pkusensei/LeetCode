using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    public void TestMethod1()
    {
        int[] nums = [1, 5, 5, 4, 11];
        int[][] edges = [[0, 1], [1, 2], [1, 3], [3, 4]];
        Assert.AreEqual(9, sol.MinimumScore(nums, edges));
    }

    [TestMethod]
    public void TestMethod2()
    {
        int[] nums = [5, 5, 2, 4, 4, 2];
        int[][] edges = [[0, 1], [1, 2], [5, 2], [4, 3], [1, 3]];
        Assert.AreEqual(0, sol.MinimumScore(nums, edges));
    }

    [TestMethod]
    public void TestMethod3()
    {
    }

    [TestMethod]
    public void TestMethod4()
    {
    }
}