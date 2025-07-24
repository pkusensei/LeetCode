using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    [DataRow("One Million Two Hundred Thirty Four Thousand Five Hundred Sixty Seven", 1234567)]
    public void TestMethod1(string exp, int num)
    {
        Assert.AreEqual(exp, sol.NumberToWords(num));
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