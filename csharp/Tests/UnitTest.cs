using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    [DataRow(new[] { 2, 3, 4, 6 }, 8)]
    [DataRow(new[] { 1, 2, 4, 5, 10 }, 16)]
    public void TestMethod1(int[] nums, int exp)
    {
        Assert.AreEqual(exp, sol.TupleSameProduct(nums));
    }

    [TestMethod]
    public void TestMethod2()
    {
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