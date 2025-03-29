using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    [DataRow(new[] { 4, 9, 6, 10 }, true)]
    [DataRow(new[] { 6, 8, 11, 12 }, true)]
    [DataRow(new[] { 5, 8, 3 }, false)]
    public void TestMethod1(int[] nums, bool exp)
    {
        Assert.AreEqual(exp, sol.PrimeSubOperation(nums));
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