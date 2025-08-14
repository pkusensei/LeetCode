using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    [DataRow(7, new[] { 2, 4, 6, 8, 10 })]
    [DataRow(16, new[] { 7, 7, 7, 7, 7 })]
    public void TestMethod1(int exp, int[] nums)
    {
        Assert.AreEqual(exp, sol.NumberOfArithmeticSlices(nums));
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