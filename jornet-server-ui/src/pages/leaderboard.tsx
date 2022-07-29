import { Component } from "react";
import { Col, Container, Row, Table } from "react-bootstrap";
import { useParams } from "react-router-dom";


type Score = {
    score: number,
    meta?: string,
    timestamp: string,
    player: string,
}

type LeaderboardProps = {
    leaderboardId?: string,
};
type LeaderboardState = {
    scores: Score[],
};


class LeaderboardInner extends Component<LeaderboardProps, LeaderboardState> {
    state: LeaderboardState = {
        scores: []
    };
    componentDidMount() {
        fetch("/api/scores/" + this.props.leaderboardId)
            .then(response => response.json())
            .then(data => {
                this.setState({ scores: data });
            });
    }

    render() {
        return (
            <Container fluid="lg">
                <Row>
                    <Col>&nbsp;</Col>
                </Row>
                <Row>
                    <Col>
                        <Table striped bordered hover>
                            <thead>
                                <tr>
                                    <th>Score</th>
                                    <th>Player</th>
                                    <th>Timestamp</th>
                                    <th>Meta</th>
                                </tr>
                            </thead>
                            <tbody>
                                {
                                    this.state.scores.map((score, index) => {
                                        return <tr key={index}>
                                            <td>{score.score}</td>
                                            <td>{score.player}</td>
                                            <td>{score.timestamp}</td>
                                            <td>{score.meta}</td>
                                        </tr>
                                    })
                                }
                            </tbody>
                        </Table>
                    </Col>
                </Row>
            </Container >
        );
    }
}

export default function Leaderboard(props: LeaderboardProps) {
    let params = useParams();
    return <LeaderboardInner {...props} leaderboardId={params.leaderboardId} />;
};
