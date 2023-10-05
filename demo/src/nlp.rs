use rust_bert::gpt2::GPT2Generator;
use rust_bert::pipelines::question_answering::QuestionAnsweringModel;
use rust_bert::pipelines::question_answering::QaInput;
use rust_bert::pipelines::summarization::SummarizationModel;
use rust_bert::pipelines::generation_utils::LanguageGenerator;
use rust_bert::pipelines::generation_utils::GenerateOptions;

fn main() -> anyhow::Result<()> {

    println!("\n\nSummarization:");

    let summarization_model = SummarizationModel::new(Default::default())?;
                                                        
    let input = ["Congress Averts Shutdown by Passing Temporary Spending Bill
Speaker Kevin McCarthy, who for weeks resisted partnering with Democrats, needed their support to pass a measure that would keep the federal government temporarily open. “On this vote, the yeas are 335, the nays are 91. Two-thirds being in the affirmative, the rules are suspended. The bill is passed, and without objection, the motion to reconsider is laid on the table.” [cheering] “If you have members in your conference that won’t let you vote for appropriation bills, doesn’t want an omnibus and won’t vote for a stopgap measure, so the only answer is to shut down and not pay our troops? I don’t want to be a part of that team. I want to be a part of a conservative group that wants to get things done.” “On this vote, the yeas are 88, the nays are 9. Under the previous order requiring 60 votes for the passage of this bill, the bill is passed.” “It’s been a day full of twists and turns, but the American people can breathe a sigh of relief. There will be no government shutdown. Democrats have said from the start that the only solution for avoiding a shutdown is bipartisanship. And we’re glad that Speaker McCarthy has finally heeded our message.”"];

    let output = summarization_model.summarize(&input);

    println!("{:?}", output);


    Ok(())
}
