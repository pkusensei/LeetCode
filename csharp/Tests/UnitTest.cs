using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    [DataRow(new[] { 8, 3, 9, 3, 8 }, 2, 81)]
    [DataRow(new[] { 19, 12, 14, 6, 10, 18 }, 3, 4788)]
    public void TestMethod1(int[] nums, int k, int exp)
    {
        Assert.AreEqual(exp, sol.MaximumScore(nums, k));
    }

    [TestMethod]
    public void TestMethod2()
    { }

    [TestMethod]
    public void TestMethod3()
    {
    }

    [TestMethod]
    public void TestMethod4()
    {
    }
}