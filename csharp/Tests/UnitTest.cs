using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    [DataRow(new[] { 1 }, 1, 1)]
    [DataRow(new[] { 1, 2 }, 4, -1)]
    [DataRow(new[] { 2, -1, 2 }, 3, 3)]
    public void TestMethod1(int[] nums, int k, int ans)
    {
        Assert.AreEqual(ans, sol.WithMonotonicStack(nums, k));
    }

    [TestMethod]
    public void TestMethod2()
    {

    }

    [TestMethod]
    public void TestMethod3()
    {

    }
}